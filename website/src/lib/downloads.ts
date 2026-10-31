import {createHash} from 'node:crypto';
export const targets=[
 {id:'macos-arm64',name:'macOS',architecture:'Apple Silicon',triple:'aarch64-apple-darwin',symbol:'M'},
 {id:'macos-x64',name:'macOS',architecture:'Intel',triple:'x86_64-apple-darwin',symbol:'M'},
 {id:'linux-x64',name:'Linux',architecture:'x86-64',triple:'x86_64-unknown-linux-gnu',symbol:'L'},
 {id:'windows-x64',name:'Windows',architecture:'x86-64',triple:'x86_64-pc-windows-msvc',symbol:'W'},
] as const;
export const releases=[
 {id:'preview',name:'Preview',version:'0.1.0-preview.3',date:'28 October 2026',description:'The sample preview release.'},
 {id:'nightly',name:'Nightly',version:'0.1.0-nightly.20261031',date:'31 October 2026',description:'The sample development build.'},
] as const;
function crc32(bytes:Buffer){let crc=0xffffffff;for(const byte of bytes){crc^=byte;for(let i=0;i<8;i++)crc=(crc>>>1)^((crc&1)?0xedb88320:0);}return(crc^0xffffffff)>>>0;}
// A small stored ZIP keeps the preview download functional without dependencies.
// Every archive names its sample status in both README and machine-readable metadata.
function zip(files:{name:string;data:string}[]){
 const local:Buffer[]=[],central:Buffer[]=[];let offset=0;
 for(const file of files){
  const name=Buffer.from(file.name),data=Buffer.from(file.data),crc=crc32(data);
  const header=Buffer.alloc(30);header.writeUInt32LE(0x04034b50,0);header.writeUInt16LE(20,4);header.writeUInt16LE(0x5d5f,12);header.writeUInt32LE(crc,14);header.writeUInt32LE(data.length,18);header.writeUInt32LE(data.length,22);header.writeUInt16LE(name.length,26);
  local.push(header,name,data);
  const entry=Buffer.alloc(46);entry.writeUInt32LE(0x02014b50,0);entry.writeUInt16LE(20,4);entry.writeUInt16LE(20,6);entry.writeUInt16LE(0x5d5f,14);entry.writeUInt32LE(crc,16);entry.writeUInt32LE(data.length,20);entry.writeUInt32LE(data.length,24);entry.writeUInt16LE(name.length,28);entry.writeUInt32LE(offset,42);
  central.push(entry,name);offset+=header.length+name.length+data.length;
 }
 const directory=Buffer.concat(central),end=Buffer.alloc(22);end.writeUInt32LE(0x06054b50,0);end.writeUInt16LE(files.length,8);end.writeUInt16LE(files.length,10);end.writeUInt32LE(directory.length,12);end.writeUInt32LE(offset,16);
 return Buffer.concat([...local,directory,end]);
}
export const artifacts=releases.flatMap(release=>targets.map(target=>{
 const id=`cinch-${release.version}-${target.triple}.sample`;
 const manifest={schema:1,sample:true,project:'Cinch',version:release.version,channel:release.id,target:target.triple,containsCompiler:false,repository:'https://github.com/iand675/cinch'};
 const bytes=zip([{name:'README.txt',data:`Cinch website download preview\n\nThis is a SAMPLE archive for the Cinch website design.\nIt does not contain a compiler, runtime, installer, or executable.\n\nVersion: ${release.version}\nTarget: ${target.triple}\n\nFor source code: https://github.com/iand675/cinch\n`},{name:'release.json',data:JSON.stringify(manifest,null,2)+'\n'}]);
 return{id,release,target,filename:`${id}.zip`,url:`/downloads/files/${id}.zip`,sha256:createHash('sha256').update(bytes).digest('hex'),size:bytes.length,bytes};
}));
