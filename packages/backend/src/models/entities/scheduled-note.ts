import {
	Entity,
	JoinColumn,
	Column,
	ManyToOne,
	OneToOne,
	PrimaryColumn,
	Index,
	type Relation,
} from "typeorm";
import { Note } from "./note.js";
import { id } from "../id.js";
import { User } from "./user.js";

@Entity()
export class ScheduledNote {
	@PrimaryColumn(id())
	public id: string;

	@Index()
	@Column({
		...id(),
		comment: "The ID of the temporarily created note that corresponds to the schedule.",
	})
	public noteId: Note["id"];

	@Index()
	@Column(id())
	public userId: User["id"];

	@Column("timestamp without time zone")
	public scheduledAt: Date;

	//#region Relations
	@OneToOne(() => Note, {
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
