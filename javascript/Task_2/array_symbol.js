(function inside(){
    let array_1 = [Symbol(1), Symbol(2), Symbol(3), Symbol(4), Symbol(5)];
    let first = array_1.shift();

    main_fun(first, ...array_1).then((message)=>{
        console.log(message);
    }).catch((error)=>{
        console.log(error);
    });
})();

function main_fun(element, ...rest){
    let array_2 = [Symbol(6), Symbol(7), Symbol(-8), Symbol(-9)];
    array_2.unshift(element);
    array_2 = [...array_2, ...rest];
    let sum = 0;
    for(let symbol of array_2)
    {
        sum = sum + Number(symbol.description);
    }
    return new Promise((resolve, reject)=>{
        if(sum > 30){
            resolve("Sum is greater than 30");
        }else{
            reject("Sum is less than or equal to 30");
        }

    });
}