import config from "@/config/index.js";
import { initIdGenerator } from "backend-rs";

const length = Math.min(Math.max(config.cuid?.length ?? 16, 16), 24);
const fingerprint = config.cuid?.fingerprint ?? "";
initIdGenerator(length, fingerprint);
