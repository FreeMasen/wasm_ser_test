const js = import('./ser_test_lib.js');
js.then(js => {
    let results = js.run_test();
    let list = ensureListState();
    for (let entry of results.split('\n,')) {
        let li = document.createElement('li');
        let text = document.createTextNode(entry);
        li.appendChild(text);
        list.appendChild(li);
    }
    
});

function ensureListState() {
    let list = document.getElementById('wasm');
    console.log('ensureListState', list);
    if (!list) {
        console.log('creating new ul');
        list = document.createElement('ul');
        console.log('adding the id');
        list.setAttribute('id', 'result-list');
        console.log('Appending the body');
        document.body.appendChild(list);
    }
    while (list.hasChildNodes()) {
        let child = list.lastChild;
        if (!child) break;
        list.removeChild(child)
    }
    return list;
}