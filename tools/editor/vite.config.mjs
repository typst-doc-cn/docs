import { defineConfig } from "vite";
import { readFileSync, readdirSync } from "fs";
import { spawnSync } from "child_process";

// load locales/docs/**/*.toml
let translates = {};
reloadTranslates();

function reloadTranslates() {
  console.log("reloadTranslates");
  translates = {};
  const handle = (path, rel) => {
    const files = readdirSync(path, { withFileTypes: true });
    for (const dirent of files) {
      const file = dirent.name;
      if (dirent.isDirectory()) {
        handle(`${path}/${file}`, rel ? `${rel}/${file}` : file);
        continue;
      }

      const filePath = `${path}/${file}`;
      const subRel = rel ? `${rel}/${file}` : file;
      if (file.endsWith(".toml")) {
        const content = readFileSync(filePath, "utf-8");
        translates[subRel] = content;
      }
    }
  };
  handle("../../locales/docs", "");
  console.log("reloadTranslates done");
}

const devServer = (url, handler) => ({
  name: "lambda-shim",
  configureServer(server) {
    server.middlewares.use(url, async (req, res) => {
      // Convert the request to a Lambda event and call the handler.
      const url = new URL(
        `http://${process.env.HOST ?? "localhost"}${req.originalUrl}`
      );
      const event = {
        method: req.method,
        rawPath: url.pathname, // e.g. '/example/'
        body: await new Promise((resolve, reject) => {
          let data = "";
          req.on("data", (chunk) => {
            data += chunk;
          });
          req.on("end", () => resolve(data));
          req.on("error", (err) => reject(err));
        }),
      };
      const context = {};
      const response = await handler(event, context);
      res.statusCode = response.statusCode;
      res.end(response.body);
    });
  },
});

async function loadFiles(event, context) {
  if (event.rawPath !== "/api/translates/") {
    return {
      statusCode: 404,
      body: "Not Found",
    };
  }

  if (event.method !== "GET" && event.method !== "POST") {
    return {
      statusCode: 405,
      body: "Method Not Allowed",
    };
  }

  if (event.method === "GET") {
    console.log("Loading files...", event.method);
    return {
      statusCode: 200,
      body: JSON.stringify({ translates }),
    };
  }

  console.log("Storing files...", event.body);
  // cargo run --bin typst-docs-l10n --release -- save

  const cmd = spawnSync(
    "cargo",
    ["run", "--bin", "typst-docs-l10n", "--release", "--", "save"],
    {
      input: event.body,
      encoding: "utf-8",
      cwd: "../..",
    }
  );

  if (cmd.error) {
    console.error("Error running command:", cmd.error);
    return {
      statusCode: 500,
      body: {
        success: false,
        message: "Failed to run command",
        error: cmd.error.message,
      },
    };
  }

  if (cmd.status !== 0) {
    console.error("Command failed:", cmd.stderr);
    return {
      statusCode: 500,
      body: {
        success: false,
        message: "Failed to save translations",
        error: cmd.stderr,
      },
    };
  }

  console.log("Command output:", cmd.stdout);
  reloadTranslates();
  return {
    statusCode: 200,
    body: JSON.stringify({
      success: true,
      message: "Translations saved successfully",
    }),
  };
}

export default defineConfig({
  plugins: [
    // https://github.com/vitejs/vite/discussions/20025
    // Send any requests to /api to the Lambda function, as if the request came from AWS API Gateway.
    devServer("/api", loadFiles),
  ],
});
