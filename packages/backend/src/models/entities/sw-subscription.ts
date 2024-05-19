import {
	PrimaryColumn,
	Entity,
	Index,
	JoinColumn,
	Column,
	ManyToOne,
	type Relation,
} from "typeorm";
import { User } from "./user.js";
import { id } from "../id.js";

@Entity()
export class SwSubscription {
	@PrimaryColumn(id())
	public id: string;

	@Column("timestamp with time zone")
	public createdAt: Date;

	@Index()
	@Column(id())
	public userId: User["id"];

	@Column("varchar", {
		length: 512,
	})
	public endpoint: string;

	@Column("varchar", {
		length: 256,
	})
	public auth: string;

	@Column("varchar", {
		length: 128,
	})
	public publickey: string;

	@Column("boolean", {
		default: false,
	})
	public sendReadMessage: boolean;

	//#region Relations
	@ManyToOne(() => User, {
		onDelete: "CASCADE",
	})
	@JoinColumn()
	public user: Relation<User>;
	//#endregion
}
