use sleepytired as st;

fn main() {
    let x = 4;

    st::trace!("fdsfjkahdsljkaf");
    st::trace!("another trace");
    st::debug!("debugs");
    st::info!("{x:4}");
    st::warn!("warning :<");
    st::error!("hecc something has gone wrong ;;");

    st::trace!();
    st::trace!();
    st::debug!();
    st::info!();
    st::warn!();
    st::error!();

    // st::fatal!("trace");

    // println!("{x}");
}
