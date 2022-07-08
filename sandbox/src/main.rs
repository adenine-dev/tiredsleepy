use tiredsleepy as ts;

fn main() {
    let mut sm = ts::util::Slotmap::<f32>::new();

    let k = sm.insert(3.0);
    ts::info!("{}", sm.try_get(k).unwrap());
    ts::info!("{}", sm.try_get(k).unwrap());
    let v = sm.try_get_mut(k).unwrap();
    *v = 4.0;
    ts::info!("{}", sm.try_get(k).unwrap());

    let mut sm1 = ts::util::Slotmap::<u32>::new();
}
