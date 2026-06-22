export type AppMetadata = {
  name: string;
  version: string;
};

export function formatAppTitle(metadata: AppMetadata) {
  const version = metadata.version.trim();

  if (!version) {
    return metadata.name;
  }

  return `${metadata.name} ${version}`;
}

