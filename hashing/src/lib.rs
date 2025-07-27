use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let mut sum=0.0;
    for item in list.iter(){
        sum+=*item as f64;
    }
    sum/list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut new_list =list.to_owned();
    new_list.sort();
    
    // println!("{:?}", new_list);
    if new_list.len()%2==0{
        (new_list[new_list.len()/2]+new_list[new_list.len()/2-1])/2
    }else{
        new_list[new_list.len()/2]
    }

}

pub fn mode(list: &[i32]) -> i32 {
    let mut map=HashMap::new();
    for i in list.iter(){
        map.entry(i).and_modify(|val| *val+=1).or_insert(1);
    }
    let mut hh = (i32,i32);

    map.iter().for_each(|m| {
        if hh.0 < *m.1{
            hh.0=*m.1;
            hh.1=**m.0 ;
        }
    });
    hh.1
    
}
