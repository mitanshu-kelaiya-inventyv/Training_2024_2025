(function inside(){
    let array_1=[1, 2, 3, 4, 5]
    let first = array_1.shift();
    main_fun(first, ...array_1).then((message)=>{
        console.log("Resolved: " + message)
    }).catch((err)=>{
        console.log("Error: " + err);
    });
})();

function main_fun(element, ...rest){
    let array_2=[6, 7, -8, -9];
    array_2.unshift(element);
    array_2=[...array_2, ...rest]
    let sum=0;
    for(const value of array_2){
        sum=sum+value;
}

    return new Promise((resolve, reject)=>{
        if(sum>30){
            resolve("Sum is greater than 30")
        }else{
            reject("Sum is less than 30");
        }
    })
}

