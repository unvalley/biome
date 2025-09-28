/** should not generate diagnostics */
const someFunction = (condition) =>{
	if (condition) {
		const str = "foo";
		const { str: destructuredStr } = { str: "foo" };
		return {
			str,
			destructuredStr,
		};
	}
	const str = "foo";
	const { str: destructuredStr } = { str: "bar" };
	return {
		str,
		destructuredStr,
	};
}