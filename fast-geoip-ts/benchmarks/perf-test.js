const start = Date.now();
const lookup = require("../lookup");
(
  async function(){
    for(let i=0; i<process.env['A']; i++){
      let ip = [0,0,0,0].map(()=>Math.floor(Math.random()*256)).join('.');
      await lookup(ip);
    }
    console.log(Date.now()-start)
    //console.log(process.memoryUsage())
  }
)()

