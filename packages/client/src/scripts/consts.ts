import { noteVisibilities } from "firefish-js";
import type { NoteVisibility } from "../types/client-types";

export const noteVisibilitiesClient = (
	noteVisibilities as readonly NoteVisibility[]
).concat("private");
