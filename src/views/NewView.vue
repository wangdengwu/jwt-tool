<script setup lang="ts">
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {ElMessage} from "element-plus";

const encodeToken = async () => {
  if (header.value.trim() == '') {
    ElMessage.error('header不能为空');
    return
  }
  if (payload.value.trim() == '') {
    ElMessage.error('payload不能为空');
    return
  }
  if (privateKey.value.trim() == '') {
    ElMessage.error('privateKey不能为空');
    return
  }
  try {
    encodedToken.value = await invoke('encode_token', {
      header: header.value.trim(),
      payload: payload.value.trim(),
      privateKey: privateKey.value.trim()
    });
    ElMessage.success('生成成功');
  } catch (error) {
    ElMessage.error('生成失败:' + error);
  }
}

const header = ref('')
const payload = ref('')
const privateKey = ref('')
const encodedToken = ref('')
</script>

<template>
  <div class="page-container">
    <h1>生成Jwt</h1>
    <el-row :gutter="20">
      <el-col :span="12">
        <el-divider>Header</el-divider>
        <el-input
            v-model="header"
            :autosize="{ minRows: 6, maxRows: 16 }"
            type="textarea"
            placeholder="header json"
        />
        <el-divider>Payload</el-divider>
        <el-input
            v-model="payload"
            :autosize="{ minRows: 6, maxRows: 16 }"
            type="textarea"
            placeholder="payload json"
        />
        <el-divider>Private Key</el-divider>
        <el-input
            v-model="privateKey"
            :autosize="{ minRows: 6, maxRows: 16 }"
            type="textarea"
            placeholder="private key"
        />
        <el-row class="center">
          <el-button type="primary" @click="encodeToken">
            生成
          </el-button>
        </el-row>
      </el-col>
      <el-col :span="12">
        <el-divider>Jwt</el-divider>
        <el-text>{{ encodedToken }}</el-text>
      </el-col>
    </el-row>
  </div>
</template>

<style scoped>
.center {
  margin-top: 8px;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>