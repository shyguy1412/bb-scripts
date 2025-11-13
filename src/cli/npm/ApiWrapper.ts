//@ts-expect-error: untyped library
import untar from 'js-untar';

const ALLOWED_FILETYPES = [".js", ".jsx", ".ts", ".tsx", ".json", ".txt"]

const API_URL = 'https://registry.npmjs.org';

export type PackageInfo = {
  name: string,
  version: string,
  keywords: string[],
  license: string,
  _id: string,
  homepage: string,
  dist: {
    shasum: string,
    tarball: string,
    fileCount: number,
    integrity: string,
    signatures: [{
      sig: string,
      keyid: string;
    }],
    unpackedSize: number;
  },
  main: string,
  engines: { node: string; },
  exports: {
    [key: string]: string;
  },
  dependencies: {
    [key: string]: string;
  },
  gitHead: string,
  repository: {
    url: string,
    type: string,
    directory: string;
  },
  description: string,
};

export async function getPackageInfo(node_package: string, version = 'latest'): Promise<PackageInfo> {
  const response = await fetch(`${API_URL}/${node_package}/${version}`);
  if (response.ok) {
    return response.json();
  }

  throw { status: response.status, error: await response.text() };
};

export async function getPackage(package_info: PackageInfo): Promise<{ name: string, content: string; }[]> {

  const response = await fetch(package_info.dist.tarball);

  if (!response.ok) {
    throw { status: response.status, error: 'Failed to download tarball' };
  }

  const blob = await response.blob();
  const stream = blob.stream().pipeThrough<Uint8Array>(new DecompressionStream('gzip'));
  const reader = stream.getReader();


  const bytes: number[] = [];

  while (true) {
    const { done, value: chunk } = await reader.read();
    if (done) break;
    bytes.push(...chunk);
  }

  const buffer = new Uint8Array(bytes.length);
  buffer.set(bytes);

  const files = await untar(buffer.buffer);

  return files
    .map((file: any) => (file.name = file.name.replace(/.[cm]js$/, '.js'), file))
    .filter(({ name }: { name: string; }) => ALLOWED_FILETYPES.some(type => name.endsWith(`.${type}`)))
    .map(({ name, buffer }: { name: string, buffer: ArrayBuffer; }) => ({
      name: name.replace(/^package\//, ''),
      content: new TextDecoder().decode(buffer)
    }));
}