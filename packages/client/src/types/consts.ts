import { noteVisibilities } from "firefish-js";
import type { NoteVisibility } from "./client-types";

export const noteVisibilitiesClient: NoteVisibility[] = (
	noteVisibilities as readonly NoteVisibility[]
).concat("private");
