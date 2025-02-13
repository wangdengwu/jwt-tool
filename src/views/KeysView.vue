<script setup lang="ts">
import {ref} from "vue";
import {ElMessage} from "element-plus";
import {invoke} from "@tauri-apps/api/core";

const newKeys = async () => {
  try {
    let r: string = await invoke('new_keys', {
      bitSize: bitSize.value,
    });
    let jsonKeys = JSON.parse(r);
    console.log(jsonKeys);
    privateKey1.value = jsonKeys.private;
    publicKey1.value = jsonKeys.public;
    ElMessage.success('生成成功');
  } catch (error) {
    ElMessage.error('生成失败:' + error);
  }
}

const getPublic = async () => {
  try {
    publicKey2.value = await invoke('get_public', {
      privateKey: privateKey2.value,
    });
    ElMessage.success('获取成功');
  } catch (error) {
    ElMessage.error('获取失败:' + error);
  }
}

const getJwk = async () => {
  try {
    let r: string = await invoke('get_jwk', {
      publicKey: publicKey3.value,
    });
    let rj = JSON.parse(r);
    jwk.value = JSON.stringify(rj, Object.keys(rj).sort(), 2);
    ElMessage.success('转换成功');
  } catch (error) {
    ElMessage.error('转换失败:' + error);
  }
}

const bitSize = ref(1024)
const privateKey1 = ref('')
const publicKey1 = ref('')
const privateKey2 = ref('')
const publicKey2 = ref('')
const publicKey3 = ref('')
const jwk = ref('')
const datetime = ref('')
</script>

<template>
  <div class="page-container">
    <h1>密钥工具</h1>
    <el-tabs type="border-card">
      <el-tab-pane label="Rsa密钥生成">
        <el-form :inline="true">
          <el-form-item label="密钥长度：">
            <el-select v-model="bitSize" placeholder="密钥长度" style="width: 120px">
              <el-option
                  :key="1024"
                  :label="1024"
                  :value="1024"
              />
              <el-option
                  :key="2048"
                  :label="2048"
                  :value="2048"
              />
            </el-select>
          </el-form-item>
          <el-form-item>
            <el-button type="primary" @click="newKeys">
              生成
            </el-button>
          </el-form-item>
        </el-form>
        <el-divider>Private Key</el-divider>
        <el-text>{{ privateKey1 }}</el-text>
        <el-divider>Public Key</el-divider>
        <el-text>{{ publicKey1 }}</el-text>
      </el-tab-pane>
      <el-tab-pane label="私钥->公钥">
        <el-row :gutter="20">
          <el-col :span="12">
            <el-input
                v-model="privateKey2"
                :autosize="{ minRows: 6, maxRows: 16 }"
                type="textarea"
                placeholder="粘贴私钥"
            />
            <el-row class="center">
              <el-button type="primary" @click="getPublic">
                获取
              </el-button>
            </el-row>
          </el-col>
          <el-col :span="12">
            <el-divider>Public Key</el-divider>
            <el-text>{{ publicKey2 }}</el-text>
          </el-col>
        </el-row>
      </el-tab-pane>
      <el-tab-pane label="公钥->jwk">
        <el-row :gutter="20">
          <el-col :span="12">
            <el-input
                v-model="publicKey3"
                :autosize="{ minRows: 6, maxRows: 16 }"
                type="textarea"
                placeholder="粘贴公钥"
            />
            <el-row class="center">
              <el-button type="primary" @click="getJwk">
                转换
              </el-button>
            </el-row>
          </el-col>
          <el-col :span="12">
            <el-divider>Jwk</el-divider>
            <pre>{{ jwk }}</pre>
          </el-col>
        </el-row>
      </el-tab-pane>
      <el-tab-pane label="时间戳">
        <el-form :inline="true">
          <el-form-item label="日期和时间:">
            <el-date-picker
                v-model="datetime"
                type="datetime"
                value-format="X"
                placeholder="选择日期和时间"
            />
          </el-form-item>
          <el-form-item label="时间戳:">
            <el-input placeholder="填写时间戳" v-model="datetime"/>
          </el-form-item>
        </el-form>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<style scoped>
.center {
  margin-top: 8px;
  display: flex;
  justify-content: center;
  align-items: center;
}
pre {
  white-space: pre-wrap;
  word-wrap: break-word;
}
</style>