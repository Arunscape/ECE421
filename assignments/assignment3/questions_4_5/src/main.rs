mod combination;
mod tax;

fn main() {
    
    let data: [i32; 100] = [27450, 51416, 66685, 91440, 129637, 139097, 140685, 144186, 149515, 153092, 157611, 211521, 247964, 251452, 263879, 291643, 303345, 347344, 382151, 389291, 392025, 393916, 404197, 405788, 413271, 415167, 426821, 432243, 434139, 434524, 450810, 458729, 458826, 484488, 497629, 505543, 506755, 518094, 524614, 549485, 576036, 604028, 607481, 614614, 627778, 667268, 682788, 683910, 688343, 700914, 708807, 720305, 739813, 772386, 787806, 835810, 843795, 852254, 863971, 869326, 881414, 894796, 905327, 923351, 940191, 988118, 1015730, 1020738, 1036292, 1055296, 1072272, 1073329, 1088374, 1114110, 1116022, 1145368, 1186396, 1197438, 1241326, 1252729, 1255353, 1272581, 1288695, 1319459, 1345918, 1353656, 1356335, 1368978, 1377207, 1380827, 1383535, 1423236, 1440803, 1442793, 1443958, 1448272, 1461850, 1466312, 1477340, 1491741];

    for &i in data.iter() {
        println!("{}", tax::tax(i));
    }

}
