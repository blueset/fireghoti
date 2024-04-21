import { secureRndstr } from "backend-rs";

export default () => secureRndstr(16);
