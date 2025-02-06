<script setup lang="ts">
import {onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {Document, InfoFilled, Menu as IconMenu, Setting} from "@element-plus/icons-vue";
import {ElMessage} from "element-plus";

const tableData = ref([
  {name: '操作系统', value: '-'},
  {name: 'Tauri 版本', value: '-'},
  {name: '应用名称', value: '-'}
])
const showMessage = async () => {
  const message = await invoke('show_message')
  ElMessage.success(message)
}

onMounted(async () => {
  const info = await invoke('get_system_info')
  tableData.value = [
    {name: '操作系统', value: info.platform},
    {name: 'Tauri 版本', value: info.tauri_version},
    {name: '应用名称', value: info.app_name}
  ]
})
</script>

<template>
  <el-container class="layout-container">
    <el-aside class="aside-container">
      <h5 class="mb-2">Default colors</h5>
      <el-menu
          default-active="2"
          class="full-height-menu"
      >
        <el-menu-item index="1">
          <el-icon>
            <icon-menu/>
          </el-icon>
          <span>Navigator Two</span>
        </el-menu-item>
        <el-menu-item index="2" disabled>
          <el-icon>
            <document/>
          </el-icon>
          <span>Navigator Three</span>
        </el-menu-item>
        <el-menu-item index="3">
          <el-icon>
            <setting/>
          </el-icon>
          <span>Navigator Four</span>
        </el-menu-item>
      </el-menu>
    </el-aside>
    <el-main class="main-container">
      <el-button type="primary" @click="showMessage">
        <el-icon>
          <InfoFilled/>
        </el-icon>
        显示消息
      </el-button>
      <el-card class="mt-4">
        <template #header>
          <div class="card-header">
            <span>系统信息</span>
          </div>
        </template>
        <el-table :data="tableData" stripe>
          <el-table-column prop="name" label="项目"/>
          <el-table-column prop="value" label="值"/>
        </el-table>
      </el-card>

      <el-card class="mt-4">
        <template #header>
          <div class="card-header">
            <span>系统信息</span>
          </div>
        </template>
        <el-table :data="tableData" stripe>
          <el-table-column prop="name" label="项目"/>
          <el-table-column prop="value" label="值"/>
        </el-table>
      </el-card>
    </el-main>
  </el-container>
</template>

<style>
body {
  overflow: hidden;
}
</style>

<style scoped>
/* 关键样式 */
.layout-container {
  height: 100vh; /* 确保容器占满视口高度 */
  display: flex;
}

.aside-container {
  /* 侧边栏宽度建议使用固定值 */
  width: 200px !important;
  overflow: hidden;
  /* 继承父容器高度 */
  height: inherit;
}

.main-container {
  /* 主内容区自动填充剩余空间 */
  flex: 1;
  padding: 20px;
  overflow: auto;
}

.full-height-menu {
  /* 重要：让菜单占满侧边栏高度 */
  height: 100%;
}
</style>