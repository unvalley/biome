function f(a: number): void;
function f(b: string): void;
function f(a: number | string): void {}

function f(x: number): void;
function f(x: string): void;
function f(x: any): any {
  return x;
}

function f(x: number): void;
function f(x: string): void;
function f(x: any): any {
  return x;
}

function opt(xs?: number[]): void;
function opt(xs: number[], y: string): void;
function opt(...args: any[]) {}

interface I {
  f(): void;
  f(x: string): string;
  f(x: number): void;
}

interface I {
  f(): void;
  f(x: string): string;
  f(x: number): void;
}

interface I {
  f(): void;
  f(x: number): void;
}

interface I {
  f(): void;
  f(x: number, y?: number, ...z: number[]): void;
}

interface I {
  f(): void;
  f(...x: number[]): void;
}

interface I {
  f(): void;
  f(x?: number): void;
}

interface I {
  f(x?: number): void;
  f(x?: string): void;
}

interface I {
  f(x: number): void;
  f(x: string): void;
}

type T = {
  (): void;
  (x: number): void;
};

declare class C {
  constructor();
  constructor(x: number);
}

interface I {
  f(x: number);
  f(x: string | boolean);
}

interface I {
  f(x: number);
  f(x: [string, boolean]);
}

interface Generic<T> {
  y(x: T[]): void;
  y(x: T): void;
}

function f<T>(x: T[]): void;
function f<T>(x: T): void;

function f<T extends number>(x: T[]): void;
function f<T extends number>(x: T): void;

abstract class Foo {
  public abstract f(x: number): void;
  public abstract f(x: string): void;
}

interface Foo {
  'f'(x: string): void;
  'f'(x: number): void;
}

interface Foo {
  new (x: string): Foo;
  new (x: number): Foo;
}

enum Enum {
  Func = 'function',
}
interface IFoo {
  [Enum.Func](x: string): void;
  [Enum.Func](x: number): void;
}

export function f(line: number): number;
export function f(line: number, character?: number): number;

declare function f(line: number): number;
export function f(line: number, character?: number): number;

declare module 'foo' {
  export default function (foo: number): string[];
  export default function (foo: number, bar?: string): string[];
}

export default function (foo: number): string[];
export default function (foo: number, bar?: string): string[];
