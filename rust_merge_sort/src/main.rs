fn main() 
{
    let mut arr0:Vec<isize>=Vec::new();

    arr0.push(65);
    arr0.push(63);
    arr0.push(64);
    arr0.push(89);
    arr0.push(92);
    arr0.push(88);
    arr0.push(83);
    arr0.push(-1);
    arr0.push(-5);
    arr0.push(-3);
    arr0.push(0);
    println!("Before merge sort: {:?}",arr0);
    let size_of_arr0:usize= arr0.len();
    merge_sort(&mut arr0,0,size_of_arr0-1);
    println!("After merge sort: {:?}",arr0);
}


fn merge_sort(mut vec1: &mut Vec<isize>,start_index:usize,end_index:usize)
{
   
    
        
        if start_index>=end_index
        {
            return;
        }
        
            let mid:usize=(start_index + end_index)/2;
            
          {
            merge_sort(&mut vec1,start_index,mid);
          }
          {
            merge_sort(&mut vec1,mid+1,end_index);
          }
            let mut vec4:Vec<isize>=Vec::new();
            let mut i:usize=start_index;
            let mut j:usize=mid+1;
            while i<=mid && j<=end_index
            {
                if vec1[i]<=vec1[j]
                {
                    vec4.push(vec1[i]);
                    i+=1;
                }
                else 
                {
                    vec4.push(vec1[j]);
                    j+=1;
                }
            }
            while i<=mid 
            {
                
                vec4.push(vec1[i]);
                i+=1;
               
            }
            while j<=end_index
            {
                
                vec4.push(vec1[j]);
                j+=1;
                
            }
            let mut k:usize=start_index;
            let mut l:usize=0;
            while k<= end_index
            {
                vec1[k]=vec4[l];
                k+=1;
                l+=1;
            }
  
}
