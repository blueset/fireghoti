const less = -1;
const same = 0;
const more = 1;

export const compareFirefishVersions = (
	oldVersion: string,
	newVersion: string,
) => {
	if (oldVersion === newVersion) return same;

	const o = oldVersion.split("-");
	const n = newVersion.split("-");

	if (o[0] < n[0]) return more;
	if (o[0] === n[0] && o[1] == null && n[1] != null) return more;
	if (o[0] === n[0] && o[1] != null && n[1] != null && o[1] < n[1]) return more;

	return less;
};
