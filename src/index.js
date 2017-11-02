const wasm = require('./main.rs')

wasm.initialize({noExitRuntime: true}).then(module => {
  const fibonacci = module.cwrap('fibonacci', 'number', ['number']);
  console.log(`r ${fibonacci(45)}`);
  timed('rust', () => fibonacci(45));
});

function jsfibonacci(num){
  let a = 1, b = 0, temp;

  while (num >= 0){
    temp = a;
    a = a + b;
    b = temp;
    num--;
  }

  return b;
}
console.log(`j ${jsfibonacci(45)}`);
timed('js', () => jsfibonacci(45));

function timed(name, fn) {
	if(!console.time || !console.timeEnd)
		return fn();
	// warmup
	for(var i = 0; i < 10; i++)
		fn();
	console.time(name)
	for(var i = 0; i < 5000; i++)
		fn();
	console.timeEnd(name)
}
