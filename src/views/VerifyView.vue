<script setup lang="ts">
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {ElMessage} from 'element-plus';

const decodeToken = async () => {
  try {
    let r: string = await invoke('decode_token', {jwtToken: jwtToken.value.trim(), publicKey: publicKey.value.trim()});
    let rj = JSON.parse(r);
    decodedHeader.value = JSON.stringify(rj.header, Object.keys(rj.header).sort(), 2);
    decodedPayload.value = JSON.stringify(rj.payload, Object.keys(rj.payload).sort(), 2);
    if (publicKey.value != '') {
      if (rj.isValid) {
        ElMessage.success('校验成功');
      } else {
        ElMessage.warning('校验失败');
      }
    }
  } catch (error) {
    ElMessage.error('解码失败:' + error);
    decodedHeader.value = '';
    decodedPayload.value = '';
    publicKey.value = '';
  }
}

const jwtToken = ref('')
const decodedHeader = ref('')
const decodedPayload = ref('')
const publicKey = ref('')

</script>

<template>
  <div class="page-container">
    <h1>校验</h1>
    <el-row :gutter="20">
      <el-col :span="12">
        <el-input
            v-model="jwtToken"
            :autosize="{ minRows: 6, maxRows: 16 }"
            type="textarea"
            placeholder="粘贴 jwt token"
        />
        <el-row class="center">
          <el-button type="primary" @click="decodeToken">
            解码
          </el-button>
        </el-row>
      </el-col>
      <el-col :span="12">
        <el-divider>Header</el-divider>
        <pre>{{ decodedHeader }}</pre>
        <el-divider>Payload</el-divider>
        <pre>{{ decodedPayload }}</pre>

        <div v-if="decodedHeader!=''&&decodedPayload!=''">
          <el-input
              v-model="publicKey"
              :autosize="{ minRows: 6, maxRows: 16 }"
              type="textarea"
              placeholder="粘贴jwk格式g"
          />
          <el-row class="center">
            <el-button type="primary" @click="decodeToken">
              校验
            </el-button>
          </el-row>
        </div>
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
