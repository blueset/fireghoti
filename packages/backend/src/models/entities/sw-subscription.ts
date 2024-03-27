import {
	PrimaryColumn,
	Entity,
	Index,
	JoinColumn,
	Column,
	ManyToOne,
} from "typeorm";
import { User } from "./user.js";
import { id } from "../id.js";
import { AccessToken } from "./access-token.js";

@Entity()
export class SwSubscription {
	@PrimaryColumn(id())
	public id: string;

	@Column("timestamp with time zone")
	public createdAt: Date;

	@Index()
	@Column(id())
	public userId: User["id"];

	@ManyToOne((type) => User, {
		onDelete: "CASCADE",
	})
	@JoinColumn()
	public user: User | null;

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

	/**
	 * App notification app (token for), used for Mastodon API notifications
	 */
	@Index()
	@Column({
		...id(),
		nullable: true,
	})
	public appAccessTokenId: AccessToken["id"] | null;

	@ManyToOne((type) => AccessToken, {
		onDelete: "CASCADE",
	})
	@JoinColumn()
	public appAccessToken: AccessToken | null;
}
