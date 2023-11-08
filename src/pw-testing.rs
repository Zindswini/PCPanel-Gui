extern crate hidapi;

use std::io::Read;

use pipewire::{
    registry::GlobalObject,
    spa::{dict, ForeignDict, ReadableDict},
    Context, Listener, MainLoop,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mainloop = MainLoop::new()?;
    let context = Context::new(&mainloop)?;
    let core = context.connect(None)?;
    let registry = core.get_registry()?;

    let _listener = registry
        .add_listener_local()
        .global(|global| {
            parse_foreign_dict(global);
        })
        .register();

    let _coreListener = core.add_listener_local().

    mainloop.run();

    Ok(())
}

fn parse_foreign_dict(fd: &GlobalObject<ForeignDict>) {
    //println!("Foreign Dict: {:?}", fd.get("object.serial"));
    println!("Foreign Dict: {:?}", fd.props.;
}
