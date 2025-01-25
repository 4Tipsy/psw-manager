

export interface PswRecord {

  owner_id: string,
  record_id: string,
  app_ico: string | null,
  app_name: string,
  account_name: string,
  encoded_login: string,
  encoded_password: string,
  tags: string[],
  created_at: string,

}