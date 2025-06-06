/* should not generate diagnostics */
function foo1<T = (arg: any) => any>(arg: T) {}
function foo2<T = ([arg]: [any]) => any>(arg: T) {}
function foo3<T = ({ args }: { args: any }) => any>(arg: T) {}
function foo4<T = (...args: any[]) => any>(fn: T, args: any[]) {}
function foo5<T extends (...args: any[]) => any>(fn: T, args: any[]) {}
function foo6<T extends (...args: any[]) => any>(fn: T, ...args: any[]) {}
function foo7<T extends ([args]: any[]) => any>(fn: T, args: any[]) {}
function foo8<T extends ([...args]: any[]) => any>(fn: T, args: any[]) {}
function foo9<T extends ({ args }: { args: any }) => any>(fn: T, args: any) {}
function foo10<T extends (id: string, ...args: any[]) => any>(
  fn: T,
  ...args: any[]
) {}
