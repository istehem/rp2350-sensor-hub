const utils = require("./utils.js");
const logger = utils.createLogger();

const express = require("express");
app = express();

app.use("/api/", (req, res, next) => {
  if (req.method === "POST" || req.method === "PUT") {
    if (!req.is("application/json")) {
      return res
        .status(415)
        .send("Unsupported Media Type. Must be application/json.");
    }
  }
  next();
});
app.use(express.json());

let lastMeasurement = null;

app.get("/", (req, res) =>
  res.send(lastMeasurement || "There is no data yet."),
);

app.post("/api/temperature", (req, res) => {
  lastMeasurement = req.body.data;
  let data = { msg: "Measurement received.", measurement: lastMeasurement };
  logger.info(data);
  res.json(data);
});

app.listen(5000, () => logger.info(`⚡️Server is running at port: 5000`));
