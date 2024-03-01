package com.example.stosp.Screens.LogIn

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.viewModels
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

class View : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        System.out.println("Inside onCreate");
        setContent {
            System.out.println("Inside setContent");
            val viewModel = viewModels<Controller>();
            LogInCard(viewModel);
        }
    }
}


@Composable
fun LogInCard(
    viewModel: Lazy<Controller>
){
//    val state = viewModel.value.state
    val state = viewModel.value.state.collectAsState();
    System.out.println("Inside LogInCard");
    Column (
        modifier = Modifier.fillMaxSize(),
        verticalArrangement = Arrangement.Center,
        horizontalAlignment = Alignment.CenterHorizontally
    ) {
        OutlinedTextField(value = state.value.userName, onValueChange = {viewModel.value.setName(it)}, label ={ Text("User Name") }, modifier = Modifier.padding(0.dp,0.dp,0.dp,12.dp))
        OutlinedTextField(value = state.value.password, onValueChange = {viewModel.value.setPassword(it)}, label ={ Text("Password") },  modifier = Modifier.padding(0.dp,0.dp,0.dp,12.dp))
        Button(onClick = { viewModel.value.submitBtnClick() }, modifier = Modifier.padding(0.dp,0.dp,0.dp,12.dp)) {
            Text(text = "Submit")
        }
    }
}