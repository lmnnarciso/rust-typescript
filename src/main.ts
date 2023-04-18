function some(item: number | undefined): number | undefined {
  if (typeof item === "number") {
    return item * 5;
  }
  return undefined;
}
