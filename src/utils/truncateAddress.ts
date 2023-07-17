export const truncateAddress = (address: string)  => {
  return `${address.substring(0, address.indexOf('1') + 1)}...${address.substring(address.length - 8, address.length)}`
}