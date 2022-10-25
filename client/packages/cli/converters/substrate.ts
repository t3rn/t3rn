import { Keyring } from "@polkadot/keyring";
const keyring = new Keyring({ type: "sr25519" })

export const pub2Address = (pub: string, prefix: number) => {
    pub = pub.split('0x')[1]
    return keyring.encodeAddress(Uint8Array.from(Buffer.from(pub, 'hex')), prefix)
}

export const addressStringToPubKey = (address: string) => {
    return "0x" + Buffer.from(keyring.decodeAddress(address)).toString('hex')
}


console.log(addressStringToPubKey("bXiLNHM2wesdnvvsMqBRb3ybSEfkyHkSk3cBE4Yy3Qph4VgkX"))

const address1 = "0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48";
const address2 = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
// console.log(pub2Address(address1, 8655))
// console.log(pub2Address(address2, 8655))

// let counter = 0;
// for(let i = 1; i < 10000; i++) {
//     if(
//         check(address1, i) &&check(address2, i)
//     ) {
//         console.log("Found match:", i)
//     }
// }

function check(address: string, id: number) {
    try {
        return pub2Address(address1, id).substring(0, 2) === "t3"
    } catch(e) {
        // console.log(e)
        return false
    }
}