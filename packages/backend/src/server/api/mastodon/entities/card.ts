namespace MastodonEntity {
	export type Card = {
		url: string;
		title: string;
		description: string;
		type: "link" | "photo" | "video" | "rich";
		image: string | null;
		author_name: string;
		author_url: string;
		provider_name: string;
		provider_url: string;
		html: string;
		width: number;
		height: number;
		embed_url: string;
		blurhash: string | null;
		/** ISO 8901 date time string */
		published_at: string | null;
		image_description: string;
		language: string | null;
	};
	const x: Card = {
		image:
			"https://streetartsheffield.com/img-cache/twitter/peachzz-south-road-2.jpg",
		language: "en",
		provider_url: "",
		url: "https://streetartsheffield.com/gallery/peachzz-south-road",
		author_name: "",
		image_description: "",
		height: 0,
		author_url: "",
		provider_name: "Street Art Sheffield",
		description:
			"This beautiful mural by Peachzz is located at the top end of South Road next to the Walkley Library.",
		type: "link",
		published_at: null,
		blurhash: null,
		title: "\n\t\t\tPeachzz, South Road | Street Art Sheffield\n\t",
		width: 0,
		embed_url: "",
		html: "",
	};
}
