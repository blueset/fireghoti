import load from "./load.js";
import { readEnvironmentConfig } from "backend-rs";

export default load();
export const envOption = readEnvironmentConfig();
