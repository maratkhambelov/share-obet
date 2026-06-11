export * from './decodeResponse'
export * from './runEffect'

//TODO: временное решение
export const createTelegramHeaders = (
  initData: string,
) => ({
  'X-Telegram-Init-Data': initData,
})