/***************************************************************************
 * THIS TEMPLATE IS MADE BY MISKATUL ANWAR<miskatul.anwar.csecu@gmail.com> *
 *               GITHUB : https://github.com/miskatul-anwar                *
 *                                THANK YOU                                *
 ***************************************************************************/

use std::io::{stdin, BufRead};

fn rin_int() -> usize {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

fn rin_vec_int() -> Vec<usize> {
    let mut input = String::new();
    stdin().lock().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn main() {
    let n = rin_int(); // Assuming this reads a single integer

    let mut a = rin_vec_int(); // Assuming this reads a vector of integers
    let mut sum: usize = a.iter().sum();
    sum /= 2;

    let (mut i, mut ans, mut cnt) = (n - 1, 0, 0);
    a.sort_by(|a, b| b.cmp(a)); // Sort in descending order

    while i < n && ans <= sum {
        ans += a[i];
        i -= 1;
        cnt += 1;
    }

    println!("{cnt}");
}
/*
//============================================================================
//problem link :
// Name        : Twins.cpp
// Author      :
// Version     :
// Copyright   : Your copyright notice
// Description : Hello World in C++, Ansi-style
//status:accepted
//============================================================================

#include <iostream>
#include <algorithm>
using namespace std;

int main() {
     int n;
     cin>>n;
     int arr[n];
     int sum=0;
     int counter=0;
     for(int i=0;i<n;i++){
         cin>>arr[i];
         sum+=arr[i];
     }
     sum=sum/2;
     sort(arr,arr+n);
     int sum2=0;
     for(int i=n-1;i>=0;i--){
         sum2+=arr[i];
         ++counter;
        if(sum2>sum){
             break;
        }


     }

     cout<<counter;






    return 0;
}


*/
