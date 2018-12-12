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

    //Use vectors cuz #ArraysOnSteroids
    let arr_alice: Vec<i8> = input
        .split(" ")
        .map(|x| x.parse().expect("This is not an integer >:V!"))
        .collect();

    /* The input code in both cases panics! when our user inputs an invalid
    data type, I still don't know if I should use a proper Result< >
    option to handle recoverable errors */
    let mut input2 = String::new();
    println!("Please input Bob's grades in the following format:");
    println!("<int> <int> <int>");
    println!("Three integers from 1-100 separated by a space");
    io::stdin().read_line(&mut input2).expect("Input error");

    let arr_bob: Vec<i8> = input
        .split(" ")
        .map(|x| x.parse().expect("This is not an integer >:V!"))
        .collect();

    let mut a_points: i8 = 0;
    let mut b_points: i8 = 0;
    //Iterator for points add-up

    for i in 0..3 {
        if arr_alice[i] > arr_bob[i] {
            a_points += 1;
        } else if arr_alice[i] < arr_bob[i] {
            b_points += 1;
        } else if arr_alice[i] == arr_bob[i] {
            a_points = a_points;
            b_points = b_points
        }
    }
    println!("The results are the following: ");
    println!("Alice: {} | Bob: {}", a_points, b_points);
}
