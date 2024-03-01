package com.example.stosp.Screens.LogIn

import androidx.lifecycle.ViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow


// ViewModel
class Controller : ViewModel() {
    private val _state = MutableStateFlow(Model()) // Initialize with default data
    val state = _state.asStateFlow()

    fun setName(name: String) {
        _state.value = _state.value.copy(userName = name)
    }

    fun setPassword(password: String) {
        _state.value = _state.value.copy(password = password)
    }

    fun submitBtnClick(){
        // handle auth API
//        GlobalScope.launch {
//            Log.d("RESP", Backend.getQuotes().toString());
//        }


    }
}

