import { loadConfig, loadEnv } from "backend-rs";

export const config = loadConfig();
export const envOption = loadEnv();
