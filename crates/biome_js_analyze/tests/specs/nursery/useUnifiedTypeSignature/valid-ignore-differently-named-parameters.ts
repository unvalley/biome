/* should not generate diagnostics when ignoreDifferentlyNamedParameters option is enabled */

// ignoreDifferentlyNamedParameters
function f1(a: number): void;
function f1(b: string): void;
function f1(a: number | string): void {}

// ignoreDifferentlyNamedParameters
function f2(m: number): void;
function f2(v: number, u: string): void;
function f2(v: number, u?: string): void {}

// ignoreDifferentlyNamedParameters
function f3(v: boolean): number;
function f3(): string;

// ignoreDifferentlyNamedParameters
function f4(v: boolean, u: boolean): number;
function f4(v: boolean): string;

// ignoreDifferentlyNamedParameters
function f5(v: number, u?: string): void {}
function f5(v: number): void;
function f5(): string;

// ignoreDifferentlyNamedParameters
function f6(a: boolean, ...c: number[]): void;
function f6(a: boolean, ...d: string[]): void;
function f6(a: boolean, ...c: (number | string)[]): void {}

// ignoreDifferentlyNamedParameters
class C {
	constructor();
	constructor(a: number, b: number);

	f(): void;
	f(a: number, b: number): void;
	f(a?: number, d?: number): void {}
}
