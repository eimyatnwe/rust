//Question(1.1)
fn vflip(img:&[String])->Vec<String>{
    let mut v_img_data = Vec::new();
    for str_data in img.iter().rev(){
        v_img_data.push(str_data.to_string());
    }
    v_img_data
}

//Question(1.2)
fn hflip(img:&[String])->Vec<String>{
    let mut h_img_data = Vec::new();
    let max_length = img.iter().map(|d| d.len()).max().unwrap_or(0);

    for row in img.iter(){
        let row:String = row.chars().rev().collect();

        let space_count = max_length - row.len();
        let white_space = " ".repeat(space_count);

        let final_string = format!("{}{}",white_space,row);
        h_img_data.push(final_string);
    }
 h_img_data
}

//Question(2.1)
fn vcat(img1:&[String],img2:&[String])->Vec<String>{
    let mut v_cat_data = Vec::new();
    for _data in img1.iter(){
     v_cat_data.push(_data.to_string());
    }
    for _data in img2.iter(){
     v_cat_data.push(_data.to_string());
    } 
    v_cat_data
}
//Question(2.2)
fn h_cat_data(img1:&[String],img2:&[String])->Vec<String>{
    let mut h_cat_data = Vec::new();
    let max_length_1 = img1.iter().map(|s| s.len()).max().unwrap_or(0);
    let max_length_2 = img2.iter().map(|s| s.len()).max().unwrap_or(0);
    let length = max_length_1.max(max_length_2);

    for (line1, line2) in img1.iter().zip(img2.iter()) {
        let row1:String = line1.chars().collect();
        let row2:String = line2.chars().collect();
       
        let white_space = " ".repeat(length- row2.len());
        let final_string = format!("{}{}{}",row1,white_space,row2);
        h_cat_data.push(final_string);
        }    
        if img1.len() > img2.len() {
            let final_string = format!("{}", img1[img1.len()-1]);
            h_cat_data.push(final_string);
        }
        else if img1.len() < img2.len() {
            let white_space = " ".repeat(length);
            let final_string = format!("{}{}", white_space, img2[img2.len()-1]);
            h_cat_data.push(final_string);
        }
 h_cat_data
}
fn main() {
    let _data = [
        "<--".to_string(),
        "#####".to_string(),
        "<==".to_string()
    ];

    //(1.1)
    let vflip_img = vflip(&_data);
    println!("Vflip");
    for _data in vflip_img.iter(){
        println!("{}",_data);
    }

    //(1.2)
    let hflip_img = hflip(&_data);
    println!("Hflip");
    for _data in hflip_img.iter(){
        println!("{}",_data);
    }

    //(2.1)
    let v_cat_img = vcat(&_data,&_data);
    // println!("Vcat");
    for _data in v_cat_img.iter(){
        // println!("{}",_data);
    }

    //(2.2)
    let h_cat_img = h_cat_data(&_data,&_data);
    // println!("Hcat");
    for _data in h_cat_img.iter(){
        // println!("{}",_data);
    }

}

#[test]
fn test_img_flip(){
    let emp = ["".to_string(); 0];
    assert_eq!(vflip(&emp),[""; 0]);
    assert_eq!(hflip(&emp),[""; 0]);

    let _data = [
        "<--",
        "#####",
        "<=="
    ].map(|v| v.to_string());

    assert_eq!(
        vflip(&_data),[
            "<==",
            "#####",
            "<--"
        ]);
}

#[test]
fn test_img_cat(){
    let emp = ["".to_string(); 0];
    assert_eq!(vcat(&emp, &emp),[""; 0]);
    assert_eq!(h_cat_data(&emp, &emp),[""; 0]);
    let _data = [
        "<--",
        "#####",
        "<=="
    ].map(|v| v.to_string());

    assert_eq!(vcat(&emp, &_data), _data);
    assert_eq!(vcat(&_data, &emp), _data);

    assert_eq!(
        vcat(&_data, &_data),[
            "<--",
            "#####",
            "<==",
            "<--",
            "#####",
            "<=="
        ]);
    
    assert_eq!(
        h_cat_data(&_data,&_data[..2]),[
            "<--  <--",
            "##########",
            "<=="
        ]);
    
    assert_eq!(
        h_cat_data(&_data[..2],&_data),[
            "<--  <--",
            "##########",
            "     <=="
        ]);
    
}