export type ConfigFileType = 'main' | 'site' | 'custom'
export type ConfigFileStatus = 'valid' | 'invalid' | 'unknown'

export interface ConfigFile {
  name: string
  type: ConfigFileType
  path: string
  size: string
  lastModified: string
  status: ConfigFileStatus
  content?: string
}

export interface ProjectConfig {
  id: string
  name: string
  path: string
  domain: string
  port: number
  root: string
  index: string[]
  php: boolean
  ssl: boolean
  remark: string
  createdAt: string
  updatedAt: string
}

export interface ProjectTemplate {
  id: string
  name: string
  content: string
  remark: string
  createdAt: string
  updatedAt: string
} 