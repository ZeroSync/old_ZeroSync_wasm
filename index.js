const rust = import('./pkg');

export function verify(data){
    return rust.then( m => m.verify( new Uint8Array(data) ) )
        .catch(console.error);
}