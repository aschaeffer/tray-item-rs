use tray_item::{IconSource, TrayItem};
use uuid::Uuid;

fn main() {
    {
        let mut tray = TrayItem::new(
            "Tray Example",
            IconSource::Resource("accessories-calculator"),
        )
        .unwrap();

        tray.add_label(Uuid::new_v4(), "Tray Label").unwrap();

        tray.add_menu_item(Uuid::new_v4(), "Hello", || {
            println!("Hello!");
        })
        .unwrap();

        tray.add_menu_item(Uuid::new_v4(), "Quit", || {
            std::process::exit(0);
        })
        .unwrap();

        std::io::stdin().read_line(&mut String::new()).unwrap();

        let id = Uuid::new_v4();
        let id2 = id;
        tray.add_menu_item(id, "Test", move || {
            println!("Clicked on menu item {}!", id2);
        })
        .unwrap();
        println!("Added menu item {}!", id);

        std::io::stdin().read_line(&mut String::new()).unwrap();

        tray.remove(id).unwrap();
        println!("Removed menu item {}!", id);

        std::io::stdin().read_line(&mut String::new()).unwrap();
    }
    println!("Dropped tray?");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
