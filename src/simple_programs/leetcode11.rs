//PROBLEM: CONTAINER WITH MOST WATER:


//passes all test cases
pub fn container_with_most_water(height: [usize;9]) -> usize{
    let mut p1: usize = 0;
    let mut p2: usize = height.len() - 1;
    let mut area:usize;
    let mut maxarea: usize = 0;
    while p1 < p2{
        if height[p1] > height[p2]{
            area = (p2-p1) * height[p2];
            p2 = p2 - 1;

        }
        else{
            area = (p2-p1) * height[p1];
            p1 = p1 + 1;
        }
        if area > maxarea{
            maxarea = area;
        }
       
        
    }
println!("{maxarea}");
return maxarea;

}


//need to look into how to pass vec as an arg even if the data is not going to be changed.