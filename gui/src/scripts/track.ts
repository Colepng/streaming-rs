import { invoke } from '@tauri-apps/api/tauri'

class Track extends HTMLElement {
    constructor() {
        super();
        const para = this.querySelector('p');
        para?.addEventListener('click', () => {
            const id = para.dataset.id;
            const name = para.dataset.name;
            const artist = para.dataset.artist;
            const album = para.dataset.album;
            if (id !== undefined) {
                invoke('play_song', {id: id, name: name, artist: artist, album: album});
                invoke('hello').then((message) => console.log(message));
            }
        });
    }
}

customElements.define('track-custom', Track);

// searchBar.forEach((bar) => {
//     bar.addEventListener('keyup', () => {
//         invoke('search', {input: bar.value}).then((result) => {
//             if (result instanceof Array) {
//                 result.forEach((track, index) => {
//                     let temp = document.getElementById(`${index}`);
//                     if (!(null === temp)) {
//                         let children = temp.children[0];
//                         children.innerHTML = `${track.name} by ${track.artist}`;
//                     }
//                 });
//             }
//         }).catch((_) => console.log('search error'));
//     });
// });
