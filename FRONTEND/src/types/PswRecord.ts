

export interface TypedPswRecord {

  owner_id: string,
  record_id: string,
  app_ico: string | null,
  app_name: string,
  account_name: string,
  encoded_login: string,
  encoded_password: string,
  tags: string[],
  created_at: string,

  _record_type: "TYPED",
}




export interface RawPswRecord {

  owner_id: string,
  record_id: string,
  app_ico: string | null,
  app_name: string,
  raw_content: string,
  tags: string[],
  created_at: string,

  _record_type: "RAW",
}