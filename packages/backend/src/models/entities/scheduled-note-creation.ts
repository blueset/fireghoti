import {
	Entity,
	JoinColumn,
	Column,
	ManyToOne,
	PrimaryColumn,
	Index,
	type Relation,
} from "typeorm";
import { Note } from "./note.js";
import { id } from "../id.js";
import { User } from "./user.js";

@Entity()
export class ScheduledNoteCreation {
	@PrimaryColumn(id())
	public id: string;

	@Index()
	@Column({
		...id(),
		comment: "The ID of note scheduled.",
	})
	public noteId: Note["id"];

	@Index()
	@Column(id())
	public userId: User["id"];

	@Column("timestamp without time zone")
	public scheduledAt: Date;

	//#region Relations
	@ManyToOne(() => Note, {
		onDelete: "CASCADE",
	})
	@JoinColumn()
	public note: Relation<Note>;

	@ManyToOne(() => User, {
		onDelete: "CASCADE",
	})
	@JoinColumn()
	public user: Relation<User>;
	//#endregion
}
