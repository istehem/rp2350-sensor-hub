const utils = require("./utils.js");
const logger = utils.createLogger();

const express = require("express");
app = express();

app.use("/api/", (req, res, next) => {
  if (req.method === "POST" || req.method === "PUT") {
    if (!req.is("application/json")) {
      return res
        .status(415)
        .json({ msg: "Unsupported Media Type. Must be application/json." });
    }
  }
  next();
});
app.use(express.json());

let lastMeasurement = null;

app.get("/api/measurements/latest", (req, res) => {
  if (lastMeasurement) {
    return res.json(lastMeasurement);
  }
  res.status(404).json({ msg: "There is no data yet." });
});

app.post("/api/measurements", (req, res) => {
  lastMeasurement = { temperature: req.body.data, date: new Date() };
  let data = { msg: "Measurement received.", measurement: lastMeasurement };
  logger.info(data);
  res.json(data);
});

app.listen(5000, () => logger.info(`⚡️Server is running at port: 5000`));
