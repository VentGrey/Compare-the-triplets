//Compare the triplets.

/* Alice and Bob each created one problem for @HackerRank. A reviewer rates the
two challenges, the first being Alice, awarding points on a scale from 1 to 100
for three categories: problem clarity, originality and difficulty.

We define the rating for Alice's challenge to be the triplet 'a' being
a = [a[0], a[1], a[2]]

And the rating for Bob's challenge to be the triplet 'b' being
b = [b[0], b[1], b[2]]

Your task is to find their comparison points by comparing:

a[0] with b[0]
a[1] with b[1]
a[2] with b[2]

Conditions:
if a[i] > b[i] -> Alice is awarded with one point.

if a[i] < b[i] -> Bob is awarded with one point.

if a[i] == b[i] -> Neither Alice nor Bob receive a point.

*/

use std::io;

fn main() {
    let mut input = String::new();
    println!("Please input Alice's grades in the following format:");
    println!("<int> <int> <int>");
    println!("Three integers from 1-100 separated by a space");
    io::stdin().read_line(&mut input).expect("Input error");

    //Usar vectores porque #ArraysOnSteroids
    let arr_alice: Vec<i8> = input.split(" ")
    .map(|x| x.parse().expect("This is not an integer >:V!"))
    .collect();

    //El código de entrada de ambos vectores resultará en un panic si la
    //entrada resulta ser errónea. ¿Debería considerar manejar errores
    //utilizando Result?
    let mut input2 = String::new();
    println!("Please input Bob's grades in the following format:");
    println!("<int> <int> <int>");
    println!("Three integers from 1-100 separated by a space");
    io::stdin().read_line(&mut input2).expect("Input error");
    
    
    let arr_bob: Vec<i8> = input.split(" ")
    .map(|x| x.parse().expect("This is not an integer >:V!"))
    .collect();
}