# Challenges

## thread communication

Il faut essayer de parcourir les éléments d'un tableau (`hashmap`) dans un `thread` particulier et en fonction de l'absence de certains éléments, en ajouter d'autre.

L'idée que j'ai eu était de prendre un `channel`, de faire calculer par le `thread` les différents éléments qui devaient être ajoutés et de les envoyer au `thread` principal via le `channel`.

Je pensais pouvoir faire ça parce que le thread aurait travaillé avec une référence du tableau et donc je pensais pouvoir le modifier en dehors du `thread` créé.

sauf que
    1) il faut d'office faire un `move` pour rentrer avec un `tx` dans la `closure`
    2) rust ne permet pas de travailler avec une référence qui peut être modifiée ailleurs

```rust
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

fn main() {
    // create hashmap & insert shit
    let mut tab: HashMap<String, String> = HashMap::new();
    tab.insert(String::from("foo"), String::from("bite"));
    tab.insert(String::from("bar"), String::from("cul"));

    // create channel
    let (tx, rx) = mpsc::channel();
    
    // attempt to force reference in thread
    let reference: &HashMap<String, String> = &tab;
    let test = thread::spawn(move || {
        let my_key = String::from("coucou");
        if !reference.contains_key(&my_key) {
            let tuple = (String::from("coucou"), String::from("beuh"));
            tx.send(tuple).unwrap();
        }
    });
    for received in rx {
        tab.insert(received.0, received.1);
    }

    test.join().unwrap();
}
```
