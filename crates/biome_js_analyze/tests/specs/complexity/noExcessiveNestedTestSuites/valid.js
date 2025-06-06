/* should not generate diagnostics */
describe("foo", function () {
	describe("bar", function () {
		describe("baz", function () {
			describe("qux", function () {
				describe("qux", function () {
					it("should get something", () => {
						expect(getSomething()).toBe("Something");
					});
				});
			});
		});
	});
});

describe("foo", function () {
	describe("bar", function () {
		describe("baz", function () {
			describe("qux", function () {
				describe("qux", function () {
					it("should get something", () => {
						expect(getSomething()).toBe("Something");
					});
				});

				fdescribe("qux", () => {
					it("something", async () => {
						expect("something").toBe("something");
					});
				});
			});
		});
	});
});

describe("foo", () => {
	describe("bar", () => {
		it("hello", async () => {
			expect("hello").toBe("hello");
		});
	});
});

xdescribe("foo", function () {
	describe("bar", function () {
		it("something", async () => {
			expect("something").toBe("something");
		});
	});
});

describe("foo", () => {
	describe.each(["hello", "world"])("%s", (a) => {});
});

z.object({})
  .describe('')
  .describe('')
  .describe('')
  .describe('')
  .describe('')
  .describe('');

z.object({
	foo: z.object({
		foo: z.object({
			foo: z.object({
				foo: z.object({
					foo: z.object().strict().describe(),
				})
				.describe(),
			})
			.describe(),
		})
		.describe(),
	})
	.describe(),
})
.describe();
